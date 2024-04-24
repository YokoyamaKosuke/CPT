# CPT
[![License](https://img.shields.io/badge/License-CC--BY--1.0-green.svg)](https://github.com/YokoyamaKosuke/CPT\_flows/blob/main/LICENSE)
[![Version](https://shields.io/badge/Version-0.1.0-blue)](https://github.com/YokoyamaKosuke/CPT/releases/tag/v0.1.0)
[![Coverage Status](https://coveralls.io/repos/github/YokoyamaKosuke/CPT/badge.svg?branch=main)](https://coveralls.io/github/YokoyamaKosuke/CPT?branch=main)


なんでも圧縮します

## Description
zip,tar,jarなど，各種圧縮フォーマットを統一的なインターフェースで扱えるようにするものである．ソフトウェア名は，安易に圧縮ツール:Compression Tool からとってCPTにした．

## Usage
```sh
FlexPress [OPTIONS] <ARGUMENTS...>
OPTIONS
  -m, --mode <MODE>     操作モードを extract, archive, auto から選択する．デフォルトは auto.
  -d, --dest <DEST>     出力先のディレクトリを指定する．デフォルトは current directory.
  -o, --output <FILE>   アーカイブの出力ファイル．デフォルトは FlexPress.zip.
  -h, --help            helpメッセージを表示する．
ARGUMENTS
  extract mode: アーカイブファイルを展開する．
  archive mode: ファイルをアーカイブする.
  auto mode:    引数にアーカイブファイルが指定されている場合, 展開する.
                それ以外の場合, ファイルをアーカイブする．
