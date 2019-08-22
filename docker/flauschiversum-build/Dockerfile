FROM python:alpine3.7
RUN apk add --update zlib-dev jpeg-dev nodejs-npm nodejs git make openssh
 RUN apk add --no-cache --virtual .build-deps build-base linux-headers
RUN pip3 install pip --upgrade
RUN pip3 install Pillow htmlmin beautifulsoup4 python-slugify python-frontmatter markdown jinja2
RUN npm install -g less

WORKDIR /var/share/flauschiversum

CMD (git pull  || git clone --depth 1 https://gitlab.magierdinge.de/nadja/flauschiversum.git /var/share/flauschiversum/) && python main.py --dst /var/www/flauschiversum/ --cache /var/share/flauschiversum-cache/