import os
import unittest
from blogcompile.model import Article
from settings import CONTENT_PATH


class ArticleTest(unittest.TestCase):
    def test_spinnerey(self):
        path = os.path.join(CONTENT_PATH, 'posts/katzenfreundlich klettern/index.md')
        article = Article(path)
        self.assertTrue(article is Article(path))

        print(article.html)


if __name__ == '__main__':
    unittest.main()
