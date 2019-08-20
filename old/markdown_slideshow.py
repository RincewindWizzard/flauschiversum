#! /usr/bin/python3
# -*- coding: utf-8 -*-
import os
from markdown import Markdown
from markdown.extensions import Extension
from markdown.treeprocessors import Treeprocessor
from markdown.util import etree

from slugify import slugify
import settings

def resized_image(url, width=None, height=None):
  """ Creates an url to a resized version of the image which is not bigger than width x height """
  dirname = os.path.dirname(url)
  basename = os.path.basename(url)
  if width and height:
    return os.path.join(dirname, '{}x{}'.format(width, height), basename)
  elif width:
    return os.path.join(dirname, '{}x'.format(width), basename)
  elif height:
    return os.path.join(dirname, 'x{}'.format(height), basename)
  else: return url

class Slideshow(Treeprocessor, Extension):
  def extendMarkdown(self, md, md_globals):
    self.md = md
    md.treeprocessors.add('slideshow', self, '>inline')

  def run(self, doc):
    self.md.images = []
    self.foo(doc)
    # resize images
    for image in doc.findall('.//img'):
      self.md.images.append((image.get('alt'), image.get('src')))
      image.set(
        'src', 
        resized_image(
          image.get('src'), 
          width=settings.image_dimension[0], 
          height=settings.image_dimension[1]
        )
      )

  def foo(self, doc):
    delta = 1
    for ri, p in enumerate(doc.getchildren()):
      empty_slideshow = True
      slideshow = etree.Element('div')
      slideshow.set('class', 'slideshow')

      slides = etree.Element('ul')
      slides.set('class', 'rslides')
      slideshow.append(slides)

      for i, child in enumerate(p.getchildren()):
        if child.tag == 'img':
          empty_slideshow = False
          slide = etree.Element('li')
          slide.set('id', slugify(child.get('src')))
          
          center = etree.SubElement(slide, 'div')
          center.set('class', 'center')

          tooltip = etree.SubElement(center, 'div')
          tooltip.set('class', 'tooltip')

          h3 = etree.SubElement(tooltip, 'h3')
          h3.text = child.get('alt')

          a = etree.SubElement(center, 'a', href='#')
          a.append(child)

          slides.append(slide)
          p.remove(child)
      if not empty_slideshow:
        doc.insert(ri + delta, slideshow)
        delta += 1


md = Markdown(extensions=[Slideshow()])
md.images = []

def compile(text):
  md.images = []
  return md.convert(text), md.images

