# FILE SYSTEM

## Superblock
### Struct
- magic_number: __u32__ = `0xDCDC`
- disk_dimension: __u32__ = `max 16 TB` 
- block_dimension: __u16__ = `4 KB`
- number_of_blocks: __u32__ = `max 0x100000000` 
- number_of_inodes: __u32__ = `max 0x10000000`
- root_index: __u32__ = `0x20001`
- data_index: __u32__ = `0x420001`
- index_free_block: __u32__ = `indice del primo blocco libero` viene messo un semaforo dopo averlo letto e viene chiamata una funzione (asincrona) per scriverne un altro, una volta trovato il semaforo diventa verde.
- index_free_inode: __u32__ = `indice del primo inode libero` funziona nella stessa maniera di index_free_block
- timestamp: __u32__ = `data`
- version: __f32__ = `1.0`

### Funzioni
  - `init(disk_size, block_size)` -> Superblock
  - `get_data(data: &str) -> Result` restituisce un dato, questo viene identificato dall' enum SuperblockEntryType
  - `write(data: &str) -> Result` permette di scrivere sui campi index_free_node e index_free_inode
  - `check_magic_number(magic_number) -> bool` ritorna true se i magic number corrispondono

### Info
Lo spazio occupato dal superblock è di 224 bytes arrotondato ai 4 KB di un blocco.
Se una funzione cerca un blocco o un inode liberi ma non sono ancora stati trovati, allora si mettono in attesa.

## Bitmap
### Struct
- bitmap: __[u8, block_size / 8]__ = `1 = blocco occupato, 0 = blocco libero`

### Funzioni
 - `load_block(index) -> Bitmap` 
 - `save_block(index) -> Result` 
 - `find_free(Superblock)` Ricerca e scrive sul superblock l'index di un blocco libero
 - `is_free(index)`
 - `set_free(index) -> Result`
 - `set_occupated(index) -> Result`

### Info
Viene scritto a indirizzo 1, cioè dopo il superblock. Viene caricata un blocco per volta, quindi 4 KB, si controlla byte per byte se è uguale a 0xFF, se lo è si scorre al prossimo altrimenti si identifica il primo bit a 0. Se un blocco della bitmap è tutto occupato si carica quello successivo e si libera quello precedente, se ci sono state modifiche, prima di liberarlo va riscritto sul disco. L'indirizzo del blocco libero va scritto nel superblock.

## Inode
### Struct
- descriptor: __u8__ = `state (1 bit) | type (2 bit) | permission (3 bits)`
- block_index: __u32__ = `indice del primo blocco di dati`
- name: __[u8, 32]__ = `32 caratteri`
- timestamp: __u32__ = `data`

### Funzioni
- `init(type, permission, block_index, name)`
- `get_data(data: &str) -> Result`
- `is_free() -> bool`
- `set_free()`  Segna l'inode come libero
- `set_occupated()` Segna l'inode come occupato
- `set_name([u8, 32])` Scrive il nome dell'inode
- `set_permission(permission)`

### Info
Il type:
- 0 = file regolare
- 1 = directory 
- 2 e 3 in futuro link

Lo spazio occupato in totale è di 41 bytes arrotondati a `64 bytes`, metterò un inode per ogni blocco. In ogni blocco stanno 64 inode.

Quando creo un nuovo inode genero una nuova istanza della struttura inode e la scrivo a un indice

## Block
### Struct
- next_block: __u32__ = `prossimo blocco oppure 0`
- data: __[u32, 1023] = `il resto dello spazio`

### Funzioni
- `write(data[u32, 1023])`
- `read(data[u32, 1023])`
- `write_next(index)` Scrive l'indirizzo del prossimo blocco
- `get_next() -> index` Ritorna l'indirizzo del prossimo blocco

## Funzioni comuni
Servono funzioni per: 
- Caricare dal disco le strutture   (trait)
- Scrivere sul disco le strutture   (trait)
- Indirizzamento (numero blocco * dimensione blocco)


## Extra
Prevedo di implementare successivamente:
- un CRC32 per verificare l'integrità del superblock 
- copia del superblock, da un altra parte per ridondanza
- versione che crea inode non in un area specifica ma permetta di salvarli ovunque nel file in modo da sfruttare meglio lo spazio
 
`Tutti i davi sono scritti in BIG ENDIAN`


### Info
Quando creo una cartella l'inode punta a un blocco dentro il quale ci sono i puntatori agli inode dei file e delle cartelle sotto 
