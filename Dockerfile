FROM python:alpine3.7
RUN apk add --update zlib-dev jpeg-dev nodejs-npm nodejs
 RUN apk add --no-cache --virtual .build-deps build-base linux-headers && pip3 install pip --upgrade

COPY . /app
WORKDIR /app
RUN pip3 install -r requirements.txt && apk del .build-deps
RUN npm install -g less
CMD python ./main.py --dst /var/www/