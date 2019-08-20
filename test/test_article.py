import os
import unittest
from blogcompile.model import Article
from settings import CONTENT_PATH


class ArticleTest(unittest.TestCase):
    def test_spinnerey(self):
        article = Article(os.path.join(CONTENT_PATH, 'posts/spinnerey/index.md'))
        print(article.meta)


if __name__ == '__main__':
    unittest.main()
