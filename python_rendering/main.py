import dearpygui.dearpygui as dpg
import numpy as np
import default_values
import rust_generation

# TODO: Dynamic size not working properly


# def calc_mandelbrot(x, y, iterations, escape_constant):
#     constant = np.complex128(x + (y*1j))
#     nextZ = constant  # Skipping first iteration

#     for i in range(iterations):
#         nextZ = np.multiply(nextZ, nextZ) + constant
#         if np.abs(nextZ - constant) > escape_constant:
#             return i

#     return iterations


# def render_texture(progress_bar, imgSize=256, iterations=3, offset=[0, 0], step=1, escape_constant=50):
#     texture_data = []
#     c = 1

#     for i in range(0, imgSize * imgSize):
#         x = (i % imgSize - (imgSize/2))*step + offset[0]
#         y = (np.floor_divide(i, imgSize) - (imgSize/2))*step + offset[1]

#         escape_time = calc_mandelbrot(x, y, iterations, escape_constant)
#         if escape_time > iterations/2:
#             factor = ((iterations - 1)-(escape_time))/(iterations - 1)
#             factor *= 2

#             red, green, blue = factor * \
#                 (200/255), factor * (25/255), factor * (25/255)
#         else:
#             factor = ((iterations - 1)-(escape_time))/(iterations - 1)
#             factor = (factor - 0.5)*2

#             red, green, blue = (200/255), (25/255), (25/255)

#             red, green, blue = red + (factor*(-200/255)), green + (
#                 factor*(+75/255)), + (factor*(+230/255))

#         texture_data.append(red)
#         texture_data.append(green)
#         texture_data.append(blue)
#         texture_data.append(255 / 255)
#         if (i+1 == imgSize*c):
#             progress = (((i)/((imgSize*imgSize))))
#             print(
#                 f"Progress: {'%.2f' % (progress*100)}%")
#             if (progress_bar != 0):
#                 dpg.set_value(progress_bar, progress)
#             c += 1

#     return texture_data


def save_btn_callback(sender, app_data, user_data):
    iterations = dpg.get_value(user_data[0])
    x = dpg.get_value(user_data[1])
    y = dpg.get_value(user_data[2])
    step = 1/np.power(2, dpg.get_value(user_data[3]))
    escape = dpg.get_value(user_data[4])
    program = user_data[5]

    print("Rendering...")

    new_texture_data = program.generator.generate_fractal(
        program.imgSize, iterations, [x, y], step, escape, program.func, default_values.THREAD_COUNT)

    print("Image Rendered")

    dpg.set_value("texture_tag", new_texture_data)


def list_box_callback(sender):
    match dpg.get_value(sender):
        case "Mandelbrot":
            program.func = rust_generation.Generators.Mandelbrot
        case "Cubic":
            program.func = rust_generation.Generators.Cubic
        case "Cos(z) * z":
            program.func = rust_generation.Generators.Cosz


class Program:
    generator = 0
    imgSize = 0
    func = rust_generation.Generators.Mandelbrot
    item_list = ["Mandelbrot", "Cubic", "Cos(z) * z"]

    def __init__(self, imgSize):
        self.imgSize = imgSize
        self.generator = rust_generation.FractalGenerator(imgSize)

    def init(self):
        dpg.create_context()

        print("Initializing...")

        print("Rendering...")
        texture_data = self.generator.generate_fractal(
            self.imgSize, default_values.ITERATIONS, default_values.OFFSET, 1/np.power(2, default_values.ZOOM), default_values.ESCAPE_CONSTANT, self.func, default_values.THREAD_COUNT)
        print("Image Rendered")

        with dpg.texture_registry(show=False):
            dpg.add_dynamic_texture(width=self.imgSize, height=self.imgSize,
                                    default_value=texture_data, tag="texture_tag")
        dpg.create_viewport(
            title='Custom Title', width=default_values.WINDOW_SIZE[0], height=default_values.WINDOW_SIZE[1])

        with dpg.window(label="FracRendering"):
            with dpg.group(label="Options"):
                dpg.add_text("Fractal Rendering")
                list_box = dpg.add_combo(
                    items=self.item_list, callback=list_box_callback, default_value="Mandelbrot")
                x_offset_input = dpg.add_input_float(
                    label="X Offset", default_value=default_values.OFFSET[0])
                y_offset_input = dpg.add_input_float(
                    label="Y Offset", default_value=default_values.OFFSET[1])
                zoom_input = dpg.add_input_int(
                    label="Zoom", default_value=default_values.ZOOM)
                iterations_input = dpg.add_input_int(
                    label="Iterations", default_value=default_values.ITERATIONS)
                escape_input = dpg.add_input_float(
                    label="Escape Value", default_value=default_values.ESCAPE_CONSTANT)
                save_btn = dpg.add_button(label="Save Values")

            with dpg.group():
                dpg.add_image("texture_tag")

            dpg.set_item_callback(save_btn, save_btn_callback)
            dpg.set_item_user_data(
                save_btn, [iterations_input, x_offset_input, y_offset_input, zoom_input, escape_input, self])

        dpg.setup_dearpygui()

        dpg.show_viewport()

        print("Initialized")

        dpg.start_dearpygui()
        dpg.destroy_context()


program = Program(default_values.SIZE)
program.init()
