build:
	cargo build --release
	rcedit .\target\release\checkmate.exe --set-icon icon.ico
run:
# builds and runs!
	echo "make run #build and runs the project," "make build #just build the project"
	cargo build --release && cls
	rcedit .\target\release\checkmate.exe --set-icon .\icon.ico
	./target/release/checkmate.exe