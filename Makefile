run:
	python3 flauschiversum.py
debug:
	export FLASK_DEBUG=1; export FLASK_APP=flauschiversum.py; flask run

.PHONY: build
build:
	mkdir build log; cd build; wget -mnH -nv -o ../log/err.log http://localhost:5000/; true
	grep -B1 "FEHLER 404" log/err.log

upload:
	bash ftp_upload.sh 
clean:
	rm -rf build log

.PHONY: docker-run docker-build
docker-build:
	cd docker-build; docker build -t flauschiversum-build .

docker-run:
	docker run -ti flauschiversum-build
