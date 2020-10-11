FROM python:alpine3.7
RUN apk add --update zlib-dev jpeg-dev nodejs-npm nodejs git make openssh libxml2 libxslt-dev rsync bash
RUN apk add --no-cache --virtual .build-deps build-base linux-headers
RUN pip3 install pip --upgrade
RUN pip3 install Pillow htmlmin beautifulsoup4 python-slugify python-frontmatter markdown jinja2 feedgen
RUN npm install -g less
RUN ssh-keygen -t ed25519 -f /root/.ssh/git-deploy -N "" -C "git-deploy@flauschiversum.de"

ADD update.sh /update.sh
ADD ssh_config /root/.ssh/config
WORKDIR /var/share/flauschiversum
CMD bash /update.sh