<!--## Front Page Content-->
<!--`beautifulhugo` supports content on your front page. Edit `/content/_index.md` to change what appears here. Delete `/content/_index.md` if you don't want any content here.-->

# CPT

{{< figure src="/avatar-icon.png" title="" width="300px" >}}
[![License](https://img.shields.io/badge/License-CC--BY--1.0-green.svg)](https://github.com/YokoyamaKosuke/CPT\_flows/blob/main/LICENSE) 
[![Coverage Status](https://coveralls.io/repos/github/YokoyamaKosuke/CPT/badge.svg?branch=main)](https://coveralls.io/github/YokoyamaKosuke/CPT?branch=main)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/yokoyamakosuke/cpt)](https://rust-reportcard.xuri.me/report/github.com/yokoyamakosuke/cpt)
[![DOI](https://zenodo.org/badge/784019734.svg)](https://zenodo.org/doi/10.5281/zenodo.11089955)


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
