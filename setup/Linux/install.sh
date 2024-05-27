dir="/"

cp "$dir/RustPanel/install/RustPanel.service" "/etc/systemd/system/RustPanel.service"
ln -s /etc/systemd/system/RustPanel.service /etc/systemd/system/multi-user.target.wants/RustPanel.service
systemctl daemon-reload
systemctl enable RustPanel.service
systemctl start RustPanel.service
#ln -s  /RustPanel/bin/rp ./rp
export PATH=`echo $PATH | sed -e 's|:/RustPanel/bin||' -e 's|/RustPanel/bin:||' -e 's|/RustPanel/bin||'` && export PATH=$PATH:/RustPanel/bin && source /etc/profile
rp --default
rm -rf "$dir/RustPanel/install"