import unittest
from blogcompile.model import Post
import settings
import os
import hashlib

class PostTest(unittest.TestCase):
    def test_html(self):
        post = Post(os.path.join(settings.CONTENT_PATH, 'posts/der-heisse-draht/index.md'))


if __name__ == '__main__':
    unittest.main()
