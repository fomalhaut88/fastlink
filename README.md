# fastlink

Fastlink is an extremely fast URL shortening service powered by [Actix WEB](https://actix.rs/). It uses a custom and quite tricky way to store the data instead of an ordinary database, thus it provides so high performance. The service has a simple API and can be easily deployed with [Docker](https://www.docker.com/) on a single machine.

## How it works

The feature of the service is the custom database to store the links. Instead of a classic DB, it allocates a file with a fixed size on a disk. It represents a long array to store URLs with a certain (their maximum) length. Each index in the array can be represented as the short URL code according to the bytes the index consists of. To encode the index the characters `0-9`, `a-z`, `A-Z`, `+` and `-` (64 ones totally) are used. More characters in the short link are used, more space are required for the allocated file, more URL addresses can be stored by the service. This approach provides the access to each saved URL for `O(1)` by the given short link, because all we need is to decode the short link into the index and to follow it. To ensure that the whole allocated space is used to save URLs, the assigned index is not random and not a hash of the given URL (it would start rewriting the existing URLs before all empty indices are filled). Instead, it is generated from a sequence that provides a full permutation of the numbers in a [finite field](https://en.wikipedia.org/wiki/Finite_field) defined by a prime modulo (that is also the maximum number of URLs that can be stored in the file) and a generator (an arbitrary number, which powers by the modulo provides the permutation). According to the finite field theory for each prime number greater or equal to 3 it is possible to find such generator. So that this way to generate a new index on add a new URL ensures the absence of the collisions (while all the indices have not been iterated), the complexity is `O(1)` and without the need to query the database at all. On the other hand, this approach makes the configuration process more confusing.

## API

| URL | Method | Description | Example |
|:-:|:-:|---|---|
| /_/version | GET | Current version of the backend. | `curl http://localhost:8080/_/version` <br /> 200 - `0.1.0` |
| /{code} | GET | Request for the URL. | `curl http://localhost:8080/xxU+3y` <br /> 302 - `Location: url=https://github.com/fomalhaut88/fastlink` in headers
| /_/add | POST | Add a new URL to the service. | `curl http://localhost:8080/_/add -d 'url=https://github.com/fomalhaut88/fastlink'` <br /> 201 - `xxU+3y` |

## Configuration

To configure the service it is necessary to set the following environment variables.

| Variable | Description | Example |
|:-:|---|:-:|
| URL_MAX_LENGTH | Maximum length of the stored URL. **Required** | `1024` |
| DB_ORDER | Number of characters in the code of the short URL. **Required** | `3` |
| DB_PRIME | The maximum number URL to store in the database. It must be prime and less than `64^DB_ORDER`. **Required** | `262139` |
| DB_GENERATOR | Generator value for the indices (see the description). It must be between `2` and `DB_PRIME - 2` and it must be a generator of the whole multiplicative group. Valid values for some `DB_PRIME` are listed below. **Required** | `54368` |
| DB_DATA_PATH | Path to the database file. Optional. Default `db/fastlink.data`. | `db/fastlink.data` |
| DB_STATE_PATH | Path to the state file, that stores the last saved index. Optional. Default `db/fastlink.state`. | `db/fastlink.state` |
| FASTLINK_HOST | Host of the webserver. Optional. Default `127.0.0.1`. | `127.0.0.1` |
| FASTLINK_PORT | Port of the webserver. Optional. Default `8080`. | `8080` |

The table with the correct configurations.

| DB_ORDER | DB_PRIME top limit | DB_PRIME | DB_GENERATOR |
|:-:|--:|--:|--:|
| `1` | `64^1 = 64` | `61` | `26` |
| `2` | `64^2 = 4 096` | `4093` | `2722` |
| `3` | `64^3 = 262 144` | `262139` | `54368` |
| `4` | `64^4 = 16 777 216` | `16777213` | `5777726` |
| `5` | `64^5 = 1 073 741 824` | `1073741789` | `758673405` |
| `6` | `64^6 = 68 719 476 736` | `68719476731` | `56560464538` |

## Deploy in docker

To build and run the service in docker it is necessary:

1. Download the source code from the repository.
2. Create a file `.env` in the root (for example, by copying `.env-example`).
3. Set the correct environment variables: `URL_MAX_LENGTH`, `DB_ORDER`, `DB_PRIME`, `DB_GENERATOR`, other variables are optional.
4. Build: `docker build -t fastlink .`
5. Run: `docker run --name fastlink-app --env-file ./.env -p 5000:5000 --volume db:/app/db -d fastlink`
6. Check the URL `http://localhost/_/version` that will show you the version if everything is correct.
