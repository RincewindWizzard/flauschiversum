import os
REPO_DIR = os.path.dirname(__file__)
CONTENT_PATH = os.path.join(REPO_DIR, 'src')
STATIC_PATH = os.path.join(CONTENT_PATH, 'static')
STYLE_PATH = os.path.join(CONTENT_PATH, 'style')
BUILD_PATH = os.path.join(REPO_DIR, 'build')
CACHE_PATH = os.path.join(REPO_DIR, '.cache')

PAGINATION_SIZE = 5
OVERLAY_IMG = os.path.join(STATIC_PATH, 'images/Wasserzeichen.png')
IMAGE_SMALL_WIDTH = 200
IMAGE_MEDIUM_WIDTH = 700
IMAGE_LARGE_WIDTH = 1000