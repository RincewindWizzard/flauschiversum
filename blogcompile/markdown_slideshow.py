#! /usr/bin/python3
# -*- coding: utf-8 -*-
import os
from markdown import Markdown
from markdown.extensions import Extension
from markdown.treeprocessors import Treeprocessor
from markdown.util import etree

from slugify import slugify

import settings

class Slideshow(Treeprocessor, Extension):
  def extendMarkdown(self, md, md_globals):
    self.md = md
    md.treeprocessors.add('slideshow', self, '>inline')

  def run(self, doc):
    from .urls import get_url_for_image
    self.create_slideshow_container(doc)
    # resize images
    for image in doc.findall('.//img'):
      image.set(
        'src', 
        get_url_for_image(
          image.get('src'), 
          width=settings.IMAGE_MEDIUM_WIDTH,
        )
      )

  def create_slideshow_container(self, doc):
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

def convert(text):
  return md.convert(text)

