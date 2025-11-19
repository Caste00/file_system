# FILE SYSTEM

## Superblock
- magic_number: __u32__ = `0xDCDC`
- disk_dimension: __u32__ = massima `16 TB` 
- block_dimension: __u16__ = `4 KB`
- number_of_blocks: __u32__ = massimi `0x100000000` 
- number_of_inodes: __u32__ = `0x10000000`
- root_index: __u32__ = `0x20001`
- data_index: __u32__ = `0x420001`
- timestamp: __u32__ = `data`
- version: __f32__ = `1.0`
- bitmap: __[u8, 536870912]__ = `1 = blocco occupato, 0 = blocco libero`

Lo spazio occupato dal superblock è di 224 bytes arrotondato ai 4 KB di un blocco, a cui bisogna aggiungere  MB per la bitmap, il primo indirizzo libero è a (536870912 / (4*1024) = 131072 + 1 = `0x20001`)

## Inode
- descriptor: __u8__ = `type (2 bit) | state (1 bit) | permission (3 bits)`
- block_index: __u32__ = `indice del primo blocco di dati`
- name: __[u8, 32]__ = `32 caratteri`
- timestamp: __u32__ = `data`

Il type:
- 0 = file regolare
- 1 = directory 
- 2 e 3 in futuro link

Lo spazio occupato in totale è di 41 bytes arrotondati a `64 bytes`, metterò un inode per ogni 64 KB (16 TB / 64 KB = `0x10000000` circa `268 milioni` per uno spazio di `16 GB`)

## Block
- next_block: __u32__ = `prossimo blocco oppure 0`
- data: __[u32, 1023] = `il resto dello spazio`



## Extra
Prevedo di implementare successivamente:
- un CRC32 per verificare l'integrità del superblock 
- copia del superblock, da un altra parte per ridondanza
- versione che crea inode non in un area specifica ma permetta di salvarli ovunque nel file in modo da sfruttare meglio lo spazio


### Info
Quando creo una cartella l'inode punta a un blocco dentro il quale ci sono i puntatori agli inode dei file e delle cartelle sotto 