install:
	echo "Compile rust source code..."
	cargo build --release
	sudo cp target/release/gitignore_templates /usr/local/bin/

	echo "Copy modified plugin to .oh-my-zsh/plugins..."
	echo "To activate plugin enable gitignore-rust in .zshrc plugins"

	cp plugin.zsh $(HOME)/.oh-my-zsh/plugins/gitignore-rust/gitignore-rust.plogin.zsh

uninstall:
	echo "Remove binary and cache file"
	sudo rm /usr/local/bin/gitignore_templates
	rm $(HOME)/.cache/gitignore_templates
	rm $(HOME)/.oh

clean:
	echo "Remove build destination folder"
	rm -r target
