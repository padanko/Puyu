# PuyuEditor

PuyuEditorは、シンプルで軽量なターミナルベースのテキストエディタです。1MB以下のバイナリサイズで、低リソース環境でも快適に動作します。<br>
Rustで書かれており、メモリ安全性を保証します。基本的なテキスト編集機能を提供し、シンプルな操作性で直感的に使用できます。

## 特徴

- **軽量**: バイナリサイズは1MB以下で、リソース使用量も非常に少ないため、古いマシンや低リソース環境でも動作します。
- **ターミナルベース**: GUIを使用せず、ターミナルでシンプルに動作します。Xサーバーが不要で、サーバー環境や仮想マシンでも動作可能です。
- **バッファ機能**: 10スロットのバッファを使用して、複数のファイルを同時に開いて簡単に切り替えながら作業できます。
- **シンプルなキーバインド**:
  - ファイルを開く: `Ctrl+X`
  - ファイルを保存する: `Ctrl+S`
  - ファイルを閉じる: `Ctrl+Q`
  - 前のバッファーに切り替える: `Ctrl+Z`
  - つぎのバッファーに切り替える: `Ctrl+C`

## インストール

```bash
git clone https://github.com/padanko/Puyu.git
cd Puyu
make
sudo make install
```

## アンインストール

```bash
sudo make uninstall
```