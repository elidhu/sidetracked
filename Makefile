SHELL=/usr/bin/env bash

RENDER_DIR=rendered
SITE_ARTICLES_DIR=../personal/content/articles

dev-hot-reload:
	cargo watch -x run -w sidetracked

verso:
	@input_files=$$(find -E . -regex ".*\.(rs|toml)$$"); \
	verso $$input_files

_render-article:
	@make verso | recto ${RENDER_DIR} ${ARTICLE}

_update-article-in-site: _render-article
	@OUTPUT=$$(realpath ${SITE_ARTICLES_DIR}/$$(basename ${ARTICLE} .md)); \
		mkdir -p $${OUTPUT}; \
		cp ${RENDER_DIR}/${ARTICLE} $${OUTPUT}/index.md

live-reload-article:
	# Check ARTICLE variable is set
	@[[ -z "${ARTICLE}" ]] && echo "ARTICLE variable not set" && exit 1; \
	 cargo watch -s 'make _update-article-in-site' -w articles
