#![cfg(target_os = "linux")]

mod display_data;
mod image_data;

pub use image::ImageFormat;

use {
    crate::xlib::{display_data::DisplayData, image_data::ImageData},
    anyhow::bail,
    std::{convert::TryFrom, mem::MaybeUninit, os::raw::c_uint, path::Path},
    x11::xlib::{
        Success, XAllPlanes, XCreateGC, XDestroyImage, XFlush, XFreeGC, XFreePixmap, XGCValues,
        XGetImage, XGetWindowAttributes, XImage, XPutImage, XWindowAttributes, ZPixmap,
    },
};

pub struct Xlib {
    displayd: DisplayData,
}

impl Xlib {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            displayd: DisplayData::new()?,
        })
    }

    /// Writes the current wallpaper to `out_path`.
    ///
    /// If format is `None` the image format will be guessed from the path, otherwise the format
    /// specified will be used.
    pub fn get<P>(&self, out_path: P, format: Option<ImageFormat>) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let DisplayData {
            display,
            root_win,
            root_pixmap,
            ..
        } = self.displayd;

        let attrs = {
            let mut attrs = MaybeUninit::<XWindowAttributes>::uninit();
            let status = unsafe { XGetWindowAttributes(display, root_win, attrs.as_mut_ptr()) };
            // This usually returns a non Success exit status even though it works, so we'll only
            // log instead of (bail!)ing.
            if status != Success as i32 {
                log::info!(
                    "Possible error while getting the root window attributes, XGetWindowAttributes returned non Success status -> {}",
                    status
                );
            }
            unsafe { attrs.assume_init() }
        };

        let ximage = unsafe {
            XGetImage(
                display,
                root_pixmap,
                0,
                0,
                attrs.width as c_uint,
                attrs.height as c_uint,
                XAllPlanes(),
                ZPixmap,
            )
        };

        unsafe { XFreePixmap(display, root_pixmap) };
        if ximage.is_null() {
            bail!("Failed to create XImage");
        }
        let depth = unsafe { (*ximage).depth };
        if depth != 24 {
            bail!("Image depth should be 24, got {}", depth);
        }
        Self::save_ximage(ximage, out_path, format)?;
        unsafe { XDestroyImage(ximage) };

        Ok(())
    }

    /// Saves an XImage to `out_path`.
    ///
    /// If format is `None` the image format will be guessed from the path, otherwise the format
    /// specified will be used.
    fn save_ximage<P>(
        img: *mut XImage,
        out_path: P,
        format: Option<ImageFormat>,
    ) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let imaged = ImageData::try_from(img)?;
        if let Some(format) = format {
            imaged.image.save_with_format(out_path, format)?;
        } else {
            imaged.image.save(out_path)?;
        }
        Ok(())
    }

    /// Sets the image at `src_path` as the wallpaper.
    ///
    /// If format is `None` the image format will be guessed from the path and content of the file.
    pub fn set<P>(&self, src_path: P, format: Option<ImageFormat>) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let DisplayData {
            display,
            root_pixmap,
            ..
        } = self.displayd;

        let (ximage, width, height) = {
            let imaged = ImageData::new(src_path, format)?;
            let imaged = imaged.fill(&self.displayd);
            (
                imaged.to_ximage(&self.displayd)?,
                imaged.width,
                imaged.height,
            )
        };

        // Creates a Graphic Context for the root Pixmap.
        let gc = {
            let mut gc_init = default_xgc_values();
            unsafe { XCreateGC(display, root_pixmap, 0, &mut gc_init) }
        };
        if gc.is_null() {
            bail!("Failed to create an X Graphic Context for the root Pixmap");
        }
        // Draws the XImage to the root Pixmap.
        let result =
            unsafe { XPutImage(display, root_pixmap, gc, ximage, 0, 0, 0, 0, width, height) };
        if result != Success as i32 {
            bail!("Failed to put the XImage to the root Pixmap");
        }
        // Flushes the display.
        unsafe { XFlush(display) };

        unsafe { XFreePixmap(display, root_pixmap) };
        unsafe { XDestroyImage(ximage) };
        unsafe { XFreeGC(display, gc) };

        Ok(())
    }
}

fn default_xgc_values() -> XGCValues {
    XGCValues {
        foreground: Default::default(),
        background: Default::default(),
        function: Default::default(),
        plane_mask: Default::default(),
        line_width: Default::default(),
        line_style: Default::default(),
        cap_style: Default::default(),
        join_style: Default::default(),
        fill_style: Default::default(),
        fill_rule: Default::default(),
        arc_mode: Default::default(),
        tile: Default::default(),
        stipple: Default::default(),
        ts_x_origin: Default::default(),
        ts_y_origin: Default::default(),
        font: Default::default(),
        subwindow_mode: Default::default(),
        graphics_exposures: Default::default(),
        clip_x_origin: Default::default(),
        clip_y_origin: Default::default(),
        clip_mask: Default::default(),
        dash_offset: Default::default(),
        dashes: Default::default(),
    }
}
