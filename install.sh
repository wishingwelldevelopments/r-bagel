mkdir ~/.config/bagel
sudo mv bin/bgl /usr/local/bin/
sudo mv bin/sendMsg /usr/local/bin/
touch ~/.config/bagel/api_key
read -p "Firebase Realtime Database Secret Key : " api_key
echo $api_key >> ~/.config/bagel/api_key

echo "Installation Complete"
