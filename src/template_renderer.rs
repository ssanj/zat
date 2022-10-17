use crate::template_proc::Template;
use crate::shared_models::ZatActionX;

pub trait TemplateRender {
  fn render(template: Template) -> ZatActionX;
}

