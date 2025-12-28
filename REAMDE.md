# FILE SYSTEM IN RUST

Cosa manca:
- Struttura del blocco dati concatenato con le sue funzioni: 
    - load
    - save
    - write_next
    - get_next
    - scrittura dati a catena

- Allocazione completa (block + inode), mancano le funzioni per:
    - marcare blocco come occupato
    - marcare un blocco come occupato
    - aggiornare bitmap
    - aggiornare superblock 
    - scrivere inode su disco

- Non esiste il formato directory, e mancano le funzioni:
    - dir_add_entry
    - dir_remove_entry
    - dir_list
    - lookup(name) -> inode

- Creazione del ROOT filesystem, manca cioè una funzione mkdir che deve fare:
    - Scrive superblock
    - Inizializza la bitmap
    - Crea inode root
    - Crea directory root

- Per finire mancano le operazioni di alto livello:
    - fs_create_file(path)
    - fs_create_dir(path)
    - fs_read(path)
    - fs_write(path, data)
    - fs_ls(path)
