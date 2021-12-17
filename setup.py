import sys
from setuptools import setup

if sys.version_info < (3, 6):
    raise Exception("Only Python 3.6 and above is supported.")

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import Binding, RustExtension

setup_requires = ["setuptools-rust>=0.9.2"]
install_requires = []

readme = open("./README.md", "r")

setup(
    name='zarena',
    packages=['zarena'],
    version="0.1.0",
    description= 'Rust Game Collection with Reninforcement Learning gym environments. This library aims to serve the same purpose as OpenSpiel, except in Rust to make it easier to use & maintain. The current games are gato, blackjack, chess & poker texas hold'em. All of these additionally support Web Assembly. You can play gato & chess against our Artificial Intelligence at Zeti Games',
    long_description=readme.read(),
    long_description_content_type='text/markdown',
    author='zetiworld',
    author_email='zetiapp@gmail.com',
    url='https://github.com/ZetiAi/zarena',
    download_url='https://github.com/ZetiAi/zarena/releases',
    keywords=["rust", "performance", "games", "wasm", "gym-environments", "gyms", "maturin"],
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 1 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X"
    ],
    rust_extensions=[
        RustExtension("zarena.zarena", "Cargo.toml", binding=Binding.PyO3, debug=False)
    ],
    zip_safe=False,
    install_requires=["numpy=1.21.1","gym=0.21.0","six=1.16.0","gy=1.0.2","Pillow=8.4.0","tqdm=4.62.3"],
    extras_require={"dev": ["pytest", "black"]}
)