cargo zigbuild --release --target aarch64-unknown-linux-gnu
rsync -avz --rsync-path="sudo rsync" target/aarch64-unknown-linux-gnu/release/vecstore-back vecstore:/home/ubuntu/vecstore-back/
ssh vecstore 'sudo chmod +x /home/ubuntu/vecstore-back/vecstore-back && sudo systemctl restart vecstore'
