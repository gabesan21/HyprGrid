# Configuração keyd com Home Row Mods para Hyprland

## 🎯 Melhor Posicionamento para Hyprland

Para Linux/Hyprland, o layout recomendado é o **GACS** (GUI-Alt-Ctrl-Shift), que coloca os modificadores mais usados nos dedos mais fortes:

```
Mão Esquerda:        Mão Direita:
A = ALT              J = SUPER
S = SHIFT            K = CTRL
D = CTRL             L = SHIFT
F = SUPER            ; = ALT
```

**Por quê essa ordem?**
- **SUPER/CTRL** nos dedos indicadores (mais fortes e ágeis) ✅
- **SHIFT** nos dedos médios (segundo mais forte) ✅
- **ALT** nos dedos mínimos (menos usado, dedo mais fraco) ✅
- **Espelhamento** permite usar qualquer mão para modificadores ✅

## ⚙️ Configuração keyd Otimizada

Crie o arquivo `/etc/keyd/default.conf`:

```ini
[ids]
*

[main]
# Home Row Mods - Layout GACS otimizado para Hyprland
# Sintaxe: overloadt(modificador, tecla, hold_timeout)

# Mão esquerda (GACS invertido para conforto)
a = overloadt(alt, a, 200)
s = overloadt(shift, s, 200)
d = overloadt(control, d, 200)
f = overloadt(meta, f, 200)

# Mão direita (espelhado)
j = overloadt(meta, j, 200)
k = overloadt(control, k, 200)
l = overloadt(shift, l, 200)
; = overloadt(alt, ;, 200)

# Caps Lock como Escape quando pressionado, Control quando segurado
capslock = overload(control, esc)
```

## 🔧 Configuração Avançada (Recomendada)

Para evitar ativações acidentais durante digitação rápida:

```ini
[ids]
*

[main]
# Home Row Mods com proteção contra ativação acidental
# overloadi = detecta contexto de digitação para reduzir latência visual

# Mão esquerda
a = overloadi(a, overloadt(alt, a, 200), 150)
s = overloadi(s, overloadt(shift, s, 200), 150)
d = overloadi(d, overloadt(control, d, 200), 150)
f = overloadi(f, overloadt(meta, f, 200), 150)

# Mão direita
j = overloadi(j, overloadt(meta, j, 200), 150)
k = overloadi(k, overloadt(control, k, 200), 150)
l = overloadi(l, overloadt(shift, l, 200), 150)
; = overloadi(;, overloadt(alt, ;, 200), 150)

# Extras úteis
capslock = overload(control, esc)
enter = overload(shift, enter)  # Enter também pode ser Shift
space = overload(symbols, space) # Space ativa camada de símbolos
```

## 📊 Explicação dos Timeouts

- **`200ms`** (hold_timeout): tempo que você precisa segurar para ativar o modificador
- **`150ms`** (idle_timeout): tempo de "ociosidade" antes de considerar que você quer o modificador
- **`overloadi`**: inteligente para digitação - resolve como letra imediatamente se você digitar rápido

## 🚀 Instalação e Ativação no Arch Linux

### Instalação do keyd

```bash
# Instalar keyd
sudo pacman -S keyd

# Criar arquivo de configuração
sudo mkdir -p /etc/keyd
sudo vim /etc/keyd/default.conf
# Cole a configuração acima

# Recarregar configuração
sudo keyd reload

# Habilitar e iniciar o serviço
sudo systemctl enable --now keyd
```

### Integração com Hyprland

Adicione ao seu `~/.config/hypr/hyprland.conf`:

```bash
# Inicia keyd automaticamente (se ainda não estiver como serviço)
exec-once = sudo systemctl start keyd
```

## 💡 Dicas de Uso

1. **Pratique primeiro com jogos de digitação** para evitar frustração inicial
2. **Ajuste os timeouts** se sentir muitas ativações acidentais (aumente para 220-250ms)
3. **Use toques leves e rápidos** para as letras normais
4. **Segure deliberadamente** quando quiser os modificadores
5. **Teste com seus atalhos do Hyprland** mais usados primeiro

## 🎨 Exemplo de Atalhos Hyprland

Com essa configuração, você pode fazer:
- **SUPER + T** → Segurar `F` + pressionar `T`
- **CTRL + SHIFT + ESC** → Segurar `D` + `S` + pressionar `ESC`
- **SUPER + SHIFT + Q** → Segurar `J` + `L` + pressionar `Q`
- **ALT + TAB** → Segurar `A` + pressionar `TAB`

## 🔧 Comandos Úteis do keyd

```bash
# Recarregar configuração
sudo keyd reload

# Ver status do serviço
sudo systemctl status keyd

# Ver logs
sudo journalctl -u keyd -f

# Testar mapeamentos
sudo keyd monitor
```

## 📚 Alternativas ao Kanata para Arch Linux

### Top 3 Recomendações

1. **keyd** (recomendado)
   - Disponível nos repositórios oficiais do Arch
   - Funciona em X11, Wayland e console virtual
   - Daemon em nível de kernel
   - Solução system-wide simples e eficiente

2. **KMonad**
   - Inspiração original do Kanata
   - Escrito em Haskell
   - Altamente configurável
   - Oferece recursos similares ao QMK

3. **xremap**
   - Específico para Linux
   - Remapeamento ciente de aplicação
   - Inspirado nas sequências de teclas do Emacs
   - Ideal para contextos diferentes por aplicação

### Outras Opções

- **evremap** - Ótimo para CapsLock como Ctrl (segurado) e Esc (pressionado)
- **evsieve** - Ferramenta de baixo nível, funciona no Wayland
- **Input Remapper** - Interface GUI + CLI
- **kbct** - Suporta também eventos de mouse

## 🎯 Suporte a Múltiplas Camadas no keyd

O keyd suporta múltiplas camadas de forma nativa:

```ini
[ids]
*

[main]
capslock = overload(symbols, esc)
space = overload(navigation, space)

[symbols]
# Camada de símbolos
d = ~
f = /
j = {
k = }
l = [
; = ]

[navigation]
# Camada de navegação
h = left
j = down
k = up
l = right
```

**Características:**
- Múltiplas camadas ativas simultaneamente
- Formam uma pilha consultada na ordem de ativação
- Suporte a hybrid modifiers
- Oneshot modifiers disponíveis

## 📖 Recursos e Documentação

- [Documentação oficial do keyd](https://github.com/rvaiya/keyd)
- [Manual do keyd no Arch Linux](https://man.archlinux.org/man/extra/keyd/keyd.1.en)
- [Guia completo sobre Home Row Mods](https://precondition.github.io/home-row-mods)
- [Input Remap Utilities - ArchWiki](https://wiki.archlinux.org/title/Input_remap_utilities)

---

**Nota:** Esta configuração é considerada a melhor para produtividade no Hyprland por reduzir movimento dos dedos a zero! 🎯
