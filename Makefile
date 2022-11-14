default: index

index: index/_site/index.html 
	mv index/_site/index.html ~/Notebook 

index/_site/index.html: target/release/notedown index/_data/menu.yml 
	cd index; bundle exec jekyll build

target/release/notedown: src/main.rs     
	cargo build --release

index/_data/menu.yml: ~/Notebook/*/*/*/*.md
	target/release/notedown ~/Notebook > index/_data/menu.yml
