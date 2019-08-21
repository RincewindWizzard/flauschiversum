import unittest
from blogcompile.model import Post, Page, Image
from blogcompile import query
from blogcompile.query import query_images, query_pages, query_posts, paginate, filtered_dataset, pagination

@filtered_dataset(query_posts)
def print_post(post):
    return post.path[-1]

@pagination(query_images, pagesize=3)
def paginated_images(index, imgs, pagecount):
    return [ img.path[-1] for img in imgs ]

class QueryTest(unittest.TestCase):
    def test_queries(self):
        dataset = [
            Page('Page1'), Page('Page2'), Page('Page3'),
            Post('Post1'), Post('Post2'), Post('Post3'), Post('Post4'),
            Image('Image1')
        ]
        self.assertEqual(len(query.execute(query_posts, dataset)), 4)
        self.assertEqual(len(query.execute(query_pages, dataset)), 3)
        self.assertEqual(len(query.execute(query_images, dataset)), 1)

    def test_paginate(self):
        self.assertEqual(paginate(list(range(20)), 3),
                         [[0, 1, 2], [3, 4, 5], [6, 7, 8], [9, 10, 11], [12, 13, 14], [15, 16, 17], [18, 19]])

    def test_filtered_dataset(self):
        dataset = [
            Page('Page1'), Page('Page2'), Page('Page3'),
            Post('Post1'), Post('Post2'), Post('Post3'), Post('Post4'),
            Image('Image1'), Image('Image2'), Image('Image3'), Image('Image4'), Image('Image5')
        ]

        self.assertEqual(['1', '2', '3', '4'], list(print_post(dataset)))
        self.assertEqual([['1', '2', '3'], ['4', '5']], list(paginated_images(dataset)))


if __name__ == '__main__':
    unittest.main()
