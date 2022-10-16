use crate::template_proc::Template;
use crate::models::ZatAction;

pub trait TemplateRender {
  fn render(template: Template) -> ZatAction;
}

