#!/bin/bash
BINARY_FOLDER=$(pwd)/target/debug
BINARY="api-server etcd piccoloctl piccoloyaml statemanager"
LINK_FOLDER=$(pwd)/bin

if [ ! -d "$LINK_FOLDER" ]; then
	mkdir bin
fi

for exe in $BINARY
do
	EXE_PATH=$LINK_FOLDER/$exe
	if [ ! -e "$EXE_PATH" ]; then
		ln -s $BINARY_FOLDER/$exe $EXE_PATH
	fi
done
