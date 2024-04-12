SHELL=/usr/bin/env bash

RENDER_DIR=rendered
ARTICLE_DIR=articles
ARTICLE1=${ARTICLE_DIR}/2024-04-10-sidetracked-introducing-the-project-and-getting-setup.md

BLOG_DIR=../personal/content/blog

verso:
	@input_files=$$(find -E . -regex ".*\.(rs|toml)$$"); \
	verso $$input_files

render-article-1:
	@make verso | recto ${RENDER_DIR} ${ARTICLE1}

update-article-1-in-blog: render-article-1
	@OUTPUT=$$(realpath ${BLOG_DIR}/$$(basename ${ARTICLE1} .md)); \
		mkdir -p $${OUTPUT}; \
		cp ${RENDER_DIR}/${ARTICLE1} $${OUTPUT}/index.md
