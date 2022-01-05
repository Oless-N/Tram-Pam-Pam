FROM ubuntu:18.04

LABEL description="Ubuntu 18.04, Python 3, Jupyter, Kivy, Android NDK"

ENV TERM=xterm

RUN groupadd user && \
    useradd -g user -d /home/user -m user

# Install packages.
RUN apt-get update && \
    apt-get install -y \
        apt-utils && \
    apt-get install -y \
        aidl \
        autoconf \
        autogen \
        automake \
        build-essential \
        clang \
        cmake \
        freeglut3 \
        freeglut3-dev \
        git \
        libffi-dev \
        libgl1-mesa-glx \
        libncurses5-dev \
        libncursesw5-dev \
        libsdl2-dev \
        libsqlite3-dev \
        libtinfo5 \
        libtool \
        mesa-utils \
        openjdk-8-jdk \
        pkg-config \
        python3 \
        python3-pip \
        unzip \
        vim \
        wget \
        zip \
        zlib1g-dev && \
    apt-get -y autoremove && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Change user account.
USER user

ENV ANDROIDSDK=~/.buildozer/android/platform/android-sdk
ENV ANDROIDNDK=~/.buildozer/android/platform/android-ndk-r17c

# Install python packages for buildozer and kivy.
# RUN pip3 install --user \
#         buildozer \
#         cython \
#         kivy \
#         Pillow \
#         pip \
#         pygame \
#         PyQt5 \
#         PyQtWebEngine \
#         python-for-android \
#         virtualenv

# Install python packages for opencv and etc.
# RUN pip3 install --user \
#         graphviz \
#         jupyter \
#         matplotlib \
#         nbconvert \
#         notebook \
#         numpy \
#         opencv-python \
#         opencv-contrib-python \
#         scikit-learn \
#         scipy

# Create src folder.
RUN mkdir /home/user/src && \
    cd /home/user/src

COPY game /home/user/src
COPY requirements requirements
COPY buildozer.spec /home/user/src/buildozer.spec
COPY buildozer-32bit.spec /home/user/src/buildozer-32bit.spec
COPY buildozer-64bit.spec /home/user/src/buildozer-64bit.spec

RUN pip3 install -U pip setuptools wheel virtualenv gitpython
RUN pip3 install -r requirements/base.txt

RUN cd /home/user/src

RUN pip3 list 
RUN buildozer android debug deploy

WORKDIR /home/user/src

CMD ["/bin/bash"]
