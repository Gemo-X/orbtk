use layout_object::ImageSizeLayoutObject;
use render_object::ImageRenderObject;
use theme::Selector;
use widget::{Template, Widget};

/// The `ImageWidget` widget is used to draw an image. It is not interactive.
/// 
/// # Properties
/// 
/// * `Selector` - CSS selector with  element name `image`, used to request the theme of the image widget.
/// 
/// # Others
/// 
/// * `ParentType`- None.
/// * `ImageSizeLayoutObject` - Used to layout the widget.
/// * `ImageRenderObject` - Used to draw the image of the widget. 
pub struct ImageWidget;

impl Widget for ImageWidget {
    fn create() -> Template {
        Template::default()
            .with_property(Selector::from("imagewidget"))
            .with_layout_object(ImageSizeLayoutObject)
            .with_render_object(ImageRenderObject)
            .with_debug_name("ImageWidget")
    }
}