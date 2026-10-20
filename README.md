# Laboratorio 5: Relleno de Triángulos (Render Pipeline)

**Curso**: Gráficas por Computadora — Universidad del Valle de Guatemala  
**Estudiante**: Diego Andre Calderon (`sallco <dcalderon918@gmail.com>`)

---

## 1. Arquitectura del Pipeline de Renderizado

El proyecto implementa un pipeline de renderizado por software en Rust estructurado en cuatro etapas secuenciales (`vértices → triángulos → fragmentos → framebuffer`):

```
+---------------------------+
|  Procesamiento Vértices   | -> vertex_shader: transforma coordenadas homogéneas con model_matrix
+---------------------------+
              |
+---------------------------+
| Ensamblaje de Primitivas  | -> primitive_assembly: agrupa vértices transformados de 3 en 3
+---------------------------+
              |
+---------------------------+
|       Rasterización       | -> rasterize (Wireframe / Barycentric / Scanline)
+---------------------------+
              |
+---------------------------+
| Procesamiento Fragmentos  | -> fragment_shader & framebuffer.point
+---------------------------+
```

---

## 2. Modos de Dibujo y Algoritmos Implementados

### 2.1 Wireframe (`TriangleMode::Wireframe`)
Dibuja el contorno alámbrico del triángulo conectando las aristas $v_0 \to v_1$, $v_1 \to v_2$ y $v_2 \to v_0$ mediante el algoritmo de líneas de Bresenham con interpolación de color y profundidad.

### 2.2 Coordenadas Baricéntricas (`TriangleMode::FilledBarycentric`)
1. **Detección de degenerados**: Calcula el área con signo en 2D mediante producto vectorial. Si el área es nula ($< 10^{-6}$), el triángulo se descarta sin producir fragmentos.
2. **Independencia de orientación (Winding Order)**: Si el área es negativa (orden horario), se intercambian $v_1$ y $v_2$ para normalizar la orientación a antihoraria ($CCW$). Invertir el orden de los vértices no altera la figura ni la hace desaparecer.
3. **Caja envolvente (Bounding Box)**: Se determinan los límites mínimos y máximos en pantalla: $[\min(x), \max(x)]$ y $[\min(y), \max(y)]$.
4. **Muestreo en el centro del píxel**: Se evalúan las funciones de arista para el punto $(x + 0.5, y + 0.5)$.
5. **Cálculo de pesos e interpolación**: Si $w_0, w_1, w_2 \ge 0$, se normalizan las coordenadas $\lambda_i = w_i / \text{área}$ y se interpola la profundidad $z$ y el color.

### 2.3 Líneas de Barrido Horizontal / Scanline (`TriangleMode::FilledScanline`)
1. **Ordenamiento vertical**: Ordena los vértices por coordenada $y$ ascendente ($p_0.y \le p_1.y \le p_2.y$).
2. **Barrido por filas**: Recorre cada fila horizontal $y$ intersecada, evaluando en $y + 0.5$.
3. **Intersecciones de aristas**: Para cada fila, calcula la intersección en el lado largo ($p_0 \to p_2$) y en el lado correspondiente ($p_0 \to p_1$ si $y < p_1.y$, o $p_1 \to p_2$ en caso contrario).
4. **Tramo horizontal**: Determina los extremos izquierdo ($x_L$) y derecho ($x_R$), rellenando de forma continua los píxeles cuyo centro cae en el intervalo $[x_L, x_R]$ con interpolación bilineal de atributos.

---

## 3. Resultados del Benchmark y Mediciones

Mediciones tomadas en modo optimizado (`cargo run --release -- --benchmark-triangles`) sobre **100 iteraciones**, evaluando la mediana de 5 rondas:

| Caso de Prueba | Fragmentos (Baricéntrico) | Fragmentos (Scanline) | Coincidencia de Píxeles | Tiempo Baricéntrico | Tiempo Scanline | Aceleración (Scanline vs Baricéntrico) |
|---|:---:|:---:|:---:|:---:|:---:|:---:|
| **Triángulo Estándar** | 25,000 | 25,000 | **100.0%** | 43.65 ms | 5.85 ms | **7.46x más rápido** |
| **Triángulo Pequeño** (20x20) | 200 | 200 | **100.0%** | 96.90 µs | 77.05 µs | **1.26x más rápido** |
| **Triángulo Grande** (600x450) | 135,050 | 135,050 | **100.0%** | 67.76 ms | 32.22 ms | **2.10x más rápido** |
| **Borde Horizontal** | 50,000 | 50,000 | **100.0%** | 22.99 ms | 11.33 ms | **2.03x más rápido** |
| **Borde Vertical** | 45,150 | 45,150 | **100.0%** | 18.41 ms | 10.36 ms | **1.78x más rápido** |
| **Orden Invertido (CW)** | 25,000 | 25,000 | **100.0%** | 11.63 ms | 5.78 ms | **2.01x más rápido** |
| **Parcialmente Fuera** | 200,000 | 200,000 | **100.0%** | 116.73 ms | 55.24 ms | **2.11x más rápido** |
| **Colineal (Degenerado)** | 0 | 0 | **100.0%** | 891.00 ns | 882.00 ns | Inmediato (< 1 µs) |

---

## 4. Análisis Comparativo de Rendimiento

1. **Precisión Geométrica Idéntica**:  
   Ambos algoritmos produjeron exactamente la misma cantidad de fragmentos para todos los casos de prueba, demostrando que la regla de muestreo del centro del píxel $(x + 0.5, y + 0.5)$ se aplicó de manera coherente en ambos métodos.

2. **Ventaja de CPU de Scanline**:  
   En ejecución secuencial sobre CPU, **Scanline supera a Baricéntrico entre 1.8x y 7.5x**. Esto se debe a que Scanline únicamente itera sobre los píxeles que efectivamente pertenecen al interior del triángulo en cada línea horizontal, mientras que el algoritmo Baricéntrico itera sobre la totalidad de la caja envolvente rectangular ($W \times H$), calculando 3 productos vectoriales por píxel incluso para píxeles externos que finalmente se descartan.

3. **Consideraciones en GPU**:  
   Aunque Scanline es superior en CPU por evitar trabajo redundante, en GPUs modernas el enfoque Baricéntrico es el estándar de la industria gracias a que la evaluación de cajas envolventes y productos vectoriales se paraleliza masivamente a nivel de hardware (warps/SIMD) sin sufrir de branching condicional dependiente de la fila.

---

## 5. Instrucciones de Ejecución

### Visualizador Interactivo
```bash
cargo run --release
```
**Controles en la ventana**:
- Tecla `1`: Modo Wireframe (contorno)
- Tecla `2`: Modo Relleno Baricéntrico
- Tecla `3`: Modo Relleno Scanline
- `Flecha Derecha` / `Espacio`: Siguiente caso de prueba
- `Flecha Izquierda`: Caso de prueba anterior

### Modo Benchmark CLI
Para ejecutar las mediciones de rendimiento sin levantar la ventana gráfica:
```bash
cargo run --release -- --benchmark-triangles
```

### Pruebas Unitarias
```bash
cargo test
```
