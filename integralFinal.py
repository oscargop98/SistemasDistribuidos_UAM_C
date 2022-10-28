from random import uniform
from time import sleep
from mpi4py import MPI
from cmath import sinh 
import numpy as np
# import matplotlib.pyplot as plt
from numpy.random import uniform as unif

comm = MPI.COMM_WORLD

rank = comm.rank
size = comm.size
name = MPI.Get_processor_name()

shared = (rank+1)*5

#Estas 2 lineas son para graficar

c = np.linspace(0.0001,3.2)
#Funcion de la integral
f=1/(1+np.sinh(2*c)*(np.log(c))**2)

#Aqui genera numeros aleatorios espcificamente en este intervalo
lim_inf = 0.8
lim_sup = 3


#inicializamos la variable de la sumatoria
suma=0

# inicializar MPI
comm = MPI.COMM_WORLD
size = comm.Get_size()
myrank = comm.Get_rank()

if myrank == 0:
    cant_num = 10000 #Cantidad de numeros aleatorios que generamos
    #este valor es el que se compartira (MPI)
    print("Soy el proceso ",str(myrank), " de ",str(size))
    sleep(8)
    x=unif(lim_inf,lim_sup,cant_num)
    comm.send(cant_num, dest=1)
    comm.send(x, dest=1)
    resultado = comm.recv(source=1)
    print("El resultado de la integral es: ")
    #resultado = (lim_sup-lim_inf)*suma/cant_num
    print(resultado)
    
    
#los procesos esclavos calculan N muestras de la función a integrar
else:

    print("Soy el proceso ",str(myrank), " de ",str(size))
    #Barrera o candado
    # comm.barrier()
    num_recv = comm.recv(source=0)
    valx = comm.recv(source=0)
    for j in range(num_recv):
        suma = suma +1/(1+np.sinh(2*valx[j])* (np.log(valx[j]))**2)    
    resultado_proceso = (lim_sup-lim_inf)*suma/num_recv
    comm.send(resultado_proceso, dest=0)
    
    #Enviarle el resultado al proceso maestro
    # print("El resultado de la integral del proceso es: ",resultado_proceso)

# comm.reduce()


# plt.xlabel('x')
# plt.ylabel('y')
# plt.plot(c,f)
# plt.hist(x,density=True)



