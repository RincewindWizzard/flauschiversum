run:
	python3 flauschiversum.py
debug:
	export FLASK_DEBUG=1; export FLASK_APP=flauschiversum.py; flask run
build:
	mkdir build log; cd build; wget -mnH -nv -o ../log/err.log http://localhost:5000/; true
	grep -B1 "FEHLER 404" log/err.log

upload:
	bash ftp_upload.sh 
clean:
	rm -rf build log
