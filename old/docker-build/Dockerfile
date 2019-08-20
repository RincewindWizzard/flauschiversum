FROM ubuntu:latest
VOLUME ["/var/www/flauschiversum/"]

RUN apt-get -y update
RUN apt-get -y install locales git python3-lxml ncftp node-less libjpeg-dev python3-dev python3-pip python3-setuptools wget
RUN pip3 install --upgrade pip

RUN locale-gen en_US.UTF-8  
ENV LANG en_US.UTF-8  
ENV LANGUAGE en_US:en  
ENV LC_ALL en_US.UTF-8


RUN git clone https://github.com/RincewindWizzard/flauschiversum.git
RUN cd /flauschiversum; pip3  install -r requirements.txt

ADD flauschiversum-update /usr/bin/flauschiversum-update


WORKDIR /flauschiversum
CMD /usr/bin/flauschiversum-update
