#!/bin/bash

# Verifică dacă git este instalat
if ! command -v git &> /dev/null
then
    echo "Git nu este instalat! Instalează-l mai întâi."
    exit
fi

# Setează numele și email-ul utilizatorului Git (îl poți modifica după preferință)
git config --global user.name "Genix2100"
git config --global user.email "your_email@example.com"

# Creează folderul pentru proiect (dacă nu există deja)
mkdir -p YR-COIN
cd YR-COIN

# Inițializează un nou repo Git local
git init

# Adaugă remote-ul GitHub
git remote add origin https://github.com/Genix2100/YR-COIN.git

# Creează fișierul README.md
echo "# YR Coin - Blockchain Project" > README.md
echo "This project represents YR Coin, a cryptocurrency built on Solana blockchain." >> README.md

# Creează un fișier de exemplu pentru proiect (ex: token-uri, smart contract etc.)
echo "Solana token smart contract goes here" > solana_token.rs

# Adaugă fișierele în git
git add .

# Comitează fișierele
git commit -m "Initial commit with README and sample code"

# Push pe GitHub
git push -u origin master

echo "Codul a fost publicat pe GitHub!"
