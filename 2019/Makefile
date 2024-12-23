preflight: fmt test

fmt:
	pipenv run black .

test:
	pipenv run mypy day* *py
	pipenv run pytest

test-watch:
	find . -name '*py' -or -name '*txt' | entr -r -c make test

