FROM python:alpine3.7
RUN apk add --update zlib-dev jpeg-dev nodejs-npm nodejs git make openssh
 RUN apk add --no-cache --virtual .build-deps build-base linux-headers
RUN pip3 install pip --upgrade
RUN pip3 install Pillow htmlmin beautifulsoup4 python-slugify python-frontmatter markdown jinja2 && apk del .build-deps
RUN npm install -g less

COPY . /var/share/flauschiversum-build
WORKDIR /var/share/flauschiversum-build

CMD sh