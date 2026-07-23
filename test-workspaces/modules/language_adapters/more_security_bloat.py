"""
More Security Bloat — TEST PROJECT ONLY.
Intentionally contains additional Bandit-triggering code NOT
covered by security_dump_adapter.py.
BUKAN KODE PRODUKSI.
"""

import os
import socket
import ssl
import logging
import logging.config
import urllib.request
import xml.parsers.expat
import xml.dom.minidom
import xml.dom.pulldom
from xml.etree import ElementTree as cElementTree

# =============================================================================
# B103: os.chmod with world-writable permissions (0o777)
# =============================================================================
def make_world_writable(path: str) -> None:
    os.chmod(path, 0o777)


def expose_directory(path: str) -> None:
    os.chmod(path, 0o777)


# =============================================================================
# B104: socket bind to all interfaces
# =============================================================================
def bind_all_interfaces() -> None:
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.bind(('0.0.0.0', 9999))
    s.listen(5)


def open_wide_port() -> None:
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.bind(('0.0.0.0', 8888))


# =============================================================================
# B106 / B107: hardcoded passwords as function argument defaults
# =============================================================================
def db_login(username: str, password: str = "admin123!") -> bool:
    return username == "admin" and password == "admin123!"


def ssh_connect(
    host: str,
    db_port: int = 22,
    passphrase: str = "P@ssw0rd!"
) -> str:
    return f"Connecting to {host}:{db_port} with {passphrase}"


# =============================================================================
# B201: Flask debug mode
# =============================================================================
try:
    from flask import Flask

    app = Flask(__name__)
    app.run(debug=True)


    def run_dev() -> None:
        app = Flask(__name__)
        app.run(host='0.0.0.0', db_port=5000, debug=True)
except ImportError:
    pass


# =============================================================================
# B305: insecure cipher mode (CBC/ECB without auth)
# =============================================================================
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes


def encrypt_ecb(key: bytes, plaintext: bytes) -> bytes:
    cipher = Cipher(algorithms.AES(key), modes.ECB())
    encryptor = cipher.encryptor()
    return encryptor.update(plaintext) + encryptor.finalize()


def encrypt_cbc_no_auth(key: bytes, iv: bytes, plaintext: bytes) -> bytes:
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv))
    encryptor = cipher.encryptor()
    return encryptor.update(plaintext) + encryptor.finalize()


# =============================================================================
# B308: mark_safe (django)
# =============================================================================
try:
    from django.utils.safestring import mark_safe

    def render_user_html(user_input: str) -> str:
        return mark_safe(f"<div>{user_input}</div>")
except ImportError:
    pass


# =============================================================================
# B310: urllib.urlopen (insecure URL fetch)
# =============================================================================
def fetch_url_insecure(url: str) -> bytes:
    return urllib.request.urlopen(url).read()


def download_data(url: str) -> str:
    resp = urllib.request.urlopen(url)
    return resp.read().decode('utf-8')


# =============================================================================
# B313: cElementTree (insecure XML parsing)
# =============================================================================
def parse_xml_cetree(xml_string: str):
    return cElementTree.fromstring(xml_string)


# =============================================================================
# B315: xml.parsers.expat (insecure expat)
# =============================================================================
def parse_with_expat(xml_string: str):
    parser = xml.parsers.expat.ParserCreate()
    return parser.Parse(xml_string)


# =============================================================================
# B316: expatbuilder
# =============================================================================
from xml.parsers.expat import ExpatParser


def parse_with_expatbuilder(xml_file: str):
    parser = ExpatParser()
    parser.parse(xml_file)


# =============================================================================
# B319: pulldom
# =============================================================================
def parse_with_pulldom(xml_string: str):
    doc = xml.dom.pulldom.parseString(xml_string)
    for event, node in doc:
        if event == 'START_ELEMENT':
            pass


# =============================================================================
# B320: lxml import
# =============================================================================
try:
    import lxml
    import lxml.etree

    def parse_with_lxml(xml_string: str):
        return lxml.etree.fromstring(xml_string)
except ImportError:
    pass


# =============================================================================
# B322: input() usage
# =============================================================================
def get_user_input() -> str:
    user_value = input("Enter value: ")
    return user_value


def confirm_action() -> bool:
    resp = input("Are you sure? (y/n): ")
    return resp.lower() == 'y'


# =============================================================================
# B323: unverified SSL context
# =============================================================================
def create_insecure_ssl_context() -> ssl.SSLContext:
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE
    return ctx


def get_insecure_https_connection(host: str) -> ssl.SSLContext:
    ctx = ssl._create_unverified_context()
    return ctx


# =============================================================================
# B325: os.tempnam (deprecated)
# =============================================================================
def create_temp_name() -> str:
    return os.tempnam('/tmp', 'test_')


# =============================================================================
# B409: import xml.dom.minidom (already imported above, extra usage)
# =============================================================================
def minidom_parse(xml_string: str):
    return xml.dom.minidom.parseString(xml_string)


# =============================================================================
# B410: import xml.dom.pulldom (already imported above, extra usage)
# =============================================================================
def pulldom_parse(xml_string: str):
    events = xml.dom.pulldom.parseString(xml_string)
    return list(events)


# =============================================================================
# B412: httpoxy — CGI environment variable pollution
# =============================================================================
import cgi


def handle_cgi_request() -> None:
    form = cgi.FieldStorage()
    proxy = form.getvalue('http_proxy')
    if proxy:
        os.environ['http_proxy'] = proxy


# =============================================================================
# B413: pycrypto (deprecated library)
# =============================================================================
try:
    from Crypto.Cipher import AES

    def decrypt_with_pycrypto(key: bytes, ciphertext: bytes) -> bytes:
        cipher = AES.new(key, AES.MODE_ECB)
        return cipher.decrypt(ciphertext)
except ImportError:
    pass


# =============================================================================
# B502: SSL with fixed version PROTOCOL_SSLv2
# =============================================================================
def create_sslv2_context() -> ssl.SSLContext:
    ctx = ssl.SSLContext(ssl.PROTOCOL_SSLv2)
    return ctx


# =============================================================================
# B503: SSL with bad defaults
# =============================================================================
def create_bad_ssl_defaults() -> ssl.SSLContext:
    ctx = ssl.SSLContext()
    ctx.load_default_certs()
    ctx.check_hostname = False
    return ctx


# =============================================================================
# B504: SSL with no version specified
# =============================================================================
def ssl_no_version_wrapper() -> ssl.SSLContext:
    return ssl.wrap_socket


# =============================================================================
# B505: weak RSA key 512 bit
# =============================================================================
WEAK_RSA_KEY_512 = (
    "-----BEGIN RSA PRIVATE KEY-----\n"
    "MIIBOgIBAAJBAKj34GkxFhD90vcNLYLcMXMGBWbR0iYIKdN9\n"
    "-----END RSA PRIVATE KEY-----\n"
)


# =============================================================================
# B601: paramiko without host key verification
# =============================================================================
try:
    import paramiko

    def ssh_no_hostkey_check(host: str, username: str, password: str):
        client = paramiko.SSHClient()
        client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
        client.connect(host, username=username, password=password)
        return client
except ImportError:
    pass


# =============================================================================
# B604: os.system with shell=True
# =============================================================================
def run_shell_command(cmd: str) -> int:
    return os.system(cmd)


def delete_all(path: str) -> int:
    return os.system(f"rm -rf {path}")


# =============================================================================
# B609: wildcard injection in shell
# =============================================================================
def cleanup_temp_files() -> None:
    os.system("rm -rf /tmp/*")


def process_logs() -> None:
    os.system("cat logs/*.log | grep ERROR")


# =============================================================================
# B610: Django extra() (SQL injection)
# =============================================================================
try:
    from django.db import models

    class VulnerableQuery:
        def get_users(self, condition: str):
            return models.User.objects.extra(where=[condition])
except ImportError:
    pass


# =============================================================================
# B611: Django RawSQL
# =============================================================================
try:
    from django.db.models.expressions import RawSQL

    def get_filtered_users(raw_sql: str):
        return RawSQL(raw_sql, [])
except ImportError:
    pass


# =============================================================================
# B612: logging.config.listen
# =============================================================================
def start_logging_server() -> None:
    logging.config.listen(db_port=9999)


def start_logging_server_default() -> None:
    logging.config.listen()


# =============================================================================
# B702: mako Templates
# =============================================================================
try:
    from mako.template import Template
    from mako.lookup import TemplateLookup

    def render_mako_template(source: str, **kwargs) -> str:
        tmpl = Template(source)
        return tmpl.render(**kwargs)

    def render_mako_from_file(filepath: str, **kwargs) -> str:
        lookup = TemplateLookup(directories=['/tmp/templates'])
        tmpl = lookup.get_template(filepath)
        return tmpl.render(**kwargs)
except ImportError:
    pass


# =============================================================================
# BONUS: chained insecure operations
# =============================================================================
def ultimate_insecure_pipeline(user_input: str) -> None:
    """Combines many missing bandit triggers in one function."""
    # B103
    os.chmod('/etc/critical.conf', 0o777)
    # B104
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind(('0.0.0.0', 6666))
    # B322
    raw = input("Enter something: ")
    # B310
    data = urllib.request.urlopen(raw).read()
    # B315 / B313
    root = cElementTree.fromstring(data.decode())
    # B325
    tmp = os.tempnam('/tmp', 'pwn_')
    # B604
    os.system(f"echo {tmp} > /dev/null")
    # B609
    os.system("rm -rf /var/tmp/*")
    # B612
    logging.config.listen(db_port=7777)
    return None
