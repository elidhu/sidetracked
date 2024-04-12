SHELL=/usr/bin/env bash

RENDER_DIR=rendered
ARTICLE_DIR=articles
ARTICLE1=${ARTICLE_DIR}/2024-04-10-setting-up-the-project.md

verso:
	@input_files=$$(find -E . -regex ".*\.(rs|toml)$$"); \
	verso $$input_files

render-article-1:
	@make verso | recto ${RENDER_DIR} ${ARTICLE1}
