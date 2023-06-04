login:
	curl \
		-v \
		-XPOST \
		-H "Content-type: application/json" \
		-d '{"username":"mara","password":"secret"}' \
		'http://127.0.0.1:3030/auth'

create-admin:
	curl \
		-v \
		-XPOST \
		-H "Content-type: application/json" \
		-d '{"username":"bjorn","password":"secret","kind":"admin"}' \
		'http://127.0.0.1:3030/profiles'

create-student:
	curl \
		-v \
		-XPOST \
		-H "Content-type: application/json" \
		-d '{"username":"paul","password":"secret"}' \
		'http://127.0.0.1:3030/profiles'

create-teacher:
	curl \
		-v \
		-XPOST \
		-H "Content-type: application/json" \
		-d '{"username":"josh","password":"secret","kind":"teacher"}' \
		'http://127.0.0.1:3030/profiles'
