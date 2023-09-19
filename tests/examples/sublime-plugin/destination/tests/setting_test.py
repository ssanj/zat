import unittest

import hello_world_setting as S

# Example test with valid imports and a test to get started.
class SettingTest(unittest.TestCase):

  def test_str(self):
    settings = S.HelloWorldSetting(debug = True)
    self.assertEqual("HelloWorldSetting(debug=True)", str(settings))
