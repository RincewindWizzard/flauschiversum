all: /var/share/flauschiversum/ build

/var/share/flauschiversum/:
	cd /var/share/; git clone ssh://git@gitlab.magierdinge.de:2222/nadja/flauschiversum.git

build:
	python ./main.py --dst /var/www/
