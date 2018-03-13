#! /usr/bin/python3
# -*- coding: utf-8 -*-
import unittest
from database import database, Post

class TestPosts(unittest.TestCase):
  def setUp(self):
    self.post_path = 'src/posts/basteln/besteckschmuck/index.md'

  def test_singleton(self):
    self.assertEqual(Post(self.post_path), Post(self.post_path))

  def test_images(self):
    post = Post(self.post_path)
    self.assertEqual(post.check_images(), [])

  def test_url(self):
    post = Post(self.post_path)
    self.assertEqual(post.url, '/2016/10/besteckschmuck/')



if __name__ == '__main__':
    unittest.main()
