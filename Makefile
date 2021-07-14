install:
	echo "Compile rust source code..."
	cargo build --release
	sudo cp target/release/gitignore_templates /usr/local/bin/

	echo "Copy modified plugin to .oh-my-zsh/plugins..."
	echo "To activate plugin enable gitignore-rust in .zshrc plugins"
	touch $(HOME)/.cache/gitignore_templates
	mkdir -p $(HOME)/.oh-my-zsh/plugins/gitignore-rust/ && cp plugin.zsh $(HOME)/.oh-my-zsh/plugins/gitignore-rust/gitignore-rust.plugin.zsh

	echo "Automatically run save on install"
	gitignore_templates --save

uninstall:
	echo "Remove binary and cache file"
	sudo rm /usr/local/bin/gitignore_templates
	rm $(HOME)/.cache/gitignore_templates
	rm -r $(HOME)/.oh-my-zsh/plugins/gitignore-rust

clean:
	echo "Remove build destination folder"
	rm -r target
