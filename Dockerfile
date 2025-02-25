FROM greyltc/archlinux-aur:yay-20250223.0.362

RUN aur-install python-sel4-deps

COPY . .
CMD [""]