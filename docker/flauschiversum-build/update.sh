#!/bin/bash
GIT_REMOTE="git@git.magierdinge.de:nadja/flauschiversum"
REPO_PATH="/var/share/flauschiversum/"
CACHE_PATH="/var/share/flauschiversum-cache/"
WWW_PATH="/var/www/flauschiversum/"
FIRST_RUN_PATH="/root/.update.lock"

if [[ ! -f "$FIRST_RUN_PATH" ]]; then
    echo "Downloading the latest version of the flauschiversum sources using following key:"
    cat /root/.ssh/git-deploy.pub
    echo "Press [Enter] after adding the key to the remote hosts:"
    read
    touch $FIRST_RUN_PATH
fi



mkdir -p $REPO_PATH; cd $REPO_PATH
git pull  || \
git clone --depth 1 $GIT_REMOTE $REPO_PATH && \
python main.py --dst $WWW_PATH  --cache $CACHE_PATH

# deploy to dfl-server
rsync -avze ssh /var/www/flauschiversum/ dfl-server.magierdinge.de:/zfs/flauschiversum/