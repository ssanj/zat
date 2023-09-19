import sublime
import sublime_plugin
from typing import Optional, List
from . import hello_world_setting as SETTING
from . import settings_loader as SETTING_LOADER

class HelloWorldCommand(sublime_plugin.TextCommand):

  print("hello_world command has loaded.")

  def run(self, edit: sublime.Edit) -> None:
    if self and self.view:
      self.log("hello_world is running")
      self.settings: SETTING.HelloWorldSetting = self.load_settings()
      self.debug(f'settings: {self.settings}')

      region = self.view.sel()[0]
      self.view.replace(edit, region, "Hello World!")
    else:
      sublime.message_dialog("Could not initialise plugin")

  def is_enabled(self) -> bool:
    return True

  def is_visible(self) -> bool:
    return True

  def load_settings(self) -> SETTING.HelloWorldSetting:
    loaded_settings: sublime.Settings = sublime.load_settings('HelloWorld.sublime-settings')
    return SETTING_LOADER.SettingsLoader(loaded_settings).load()

  def log_with_context(self, message: str, context: Optional[str]):
    if context is not None:
      print(f'[HelloWorld][{context}] - {message}')
    else:
      print(f'[HelloWorld] - {message}')

  def log(self, message: str):
    self.log_with_context(message, context=None)

  def debug(self, message: str):
    if self.settings.debug:
      self.log_with_context(message, context="DEBUG")
