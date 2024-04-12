SHELL=/usr/bin/env bash

RENDER_DIR=rendered
ARTICLE_DIR=articles
ARTICLE1=${ARTICLE_DIR}/2024-04-10_sidetracked-introducing-the-project-and-getting-setup.md

SITE_ARTICLES_DIR=../personal/content/articles

dev-hot-reload:
	cargo watch -x run -w sidetracked

verso:
	@input_files=$$(find -E . -regex ".*\.(rs|toml)$$"); \
	verso $$input_files

render-article-1:
	@make verso | recto ${RENDER_DIR} ${ARTICLE1}

update-article-1-in-blog: render-article-1
	@OUTPUT=$$(realpath ${SITE_ARTICLES_DIR}/$$(basename ${ARTICLE1} .md)); \
		mkdir -p $${OUTPUT}; \
		cp ${RENDER_DIR}/${ARTICLE1} $${OUTPUT}/index.md

live-reload-article-1:
	@cargo watch -s 'make update-article-1-in-blog' -w articles
