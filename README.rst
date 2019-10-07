Echo
====

.. image:: https://gitlab.com/grauwoelfchen/echo/badges/master/pipeline.svg
   :target: https://gitlab.com/grauwoelfchen/echo/commits/master

.. image:: https://gitlab.com/grauwoelfchen/echo/badges/master/coverage.svg
   :target: https://gitlab.com/grauwoelfchen/echo/commits/master


Repository
==========

https://gitlab.com/grauwoelfchen/echo


Build
=====

Just build and run this tcp server.

.. code:: zsh

   % make build

   % ./target/debug/echo


If you want to run it in a container (e.g. Gentoo Linux).  
You may need to set HOST and PORT as you need (default: `0.0.0.0:8000`).

.. code:: zsh

   # build-arg(s) are optional
   % docker build --build-arg HOST=0.0.0.0 --build-arg PORT=5000 \
       -t grauwoelfchen/echo:latest .
   % docker container run -d -p 8000:5000 grauwoelfchen/echo:latest

See Dockerfile


Usage
=====

.. code:: zsh

   % curl -d '{"message": "Hoi Zäme!"}' http://localhost:8080
   {"message": "Hoi Zäme!"}


License
-------


.. code:: text

   Echo
   Copyright 2019 Yasuhiro Asaka

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
