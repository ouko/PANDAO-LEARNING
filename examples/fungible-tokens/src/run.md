# Publish
```cmd
resim publish .
```
calling above will provide you with component address

# once component address is present, call ininstantiate_currency function to deploy a component from this blue print

### syntex
```
resim call-function <package-address> <BluePrintName> <functionToCall> agrs1 args2 ...
```

### to call this
```
resim call-function <package-address> AnimalDigitalCurrency instantiate_currency Zebra 🦓
```
above command will deploy a coponent with given parameters, ie a digital currency with zebra name and symbol of zebra

### to call deployed components methods

resim call-method <component-address> <method_name> args1